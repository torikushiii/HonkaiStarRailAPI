const HoyoLab = require("./hoyolab");
const Defaults = require("./defaults");

const fetchAll = async () => {
	const debug = await app.Logger("debug:resolver");
	const logger = await app.Logger("resolver");

	const [hoyolab, defaults] = await Promise.all([
		HoyoLab.fetch(),
		Defaults.fetch()
	]);

	const codeData = await app.Query.collection("codes").find({}).toArray();
	const codes = new Set(codeData.map((code) => code.code));

	const data = [...hoyolab, ...defaults]
		.filter((i) => i && !i.code.toLowerCase().includes("random"))
		.sort((a, b) => {
			if (a.source === "HoyoLab Forum" && b.source !== "HoyoLab Forum") {
				return 1;
			}
			if (a.source !== "HoyoLab Forum" && b.source === "HoyoLab Forum") {
				return -1;
			}

			return 0;
		})
		.filter((i, index, self) => self.findIndex((t) => t.code === i.code) === index);

	debug.info(`Found ${data.length} codes to be checked.`);

	if (codeData.length === 0) {
		logger.warn("No codes found in database. Inserting all codes...");
		await app.Query.collection("codes").insertMany(data.map(i => ({ ...i, date: new Date(), active: false })));
		return [];
	}

	const filteredData = data.filter(i => !codes.has(i.code));
	if (filteredData.length === 0) {
		logger.info("No new codes found.");
		return [];
	}

	logger.info(`Found ${filteredData.length} new codes. Inserting...`);
	logger.info(filteredData);

	await app.Query.collection("codes").insertMany(filteredData.map(i => ({ ...i, date: new Date(), active: true })));

	return filteredData;
};

module.exports = {
	fetchAll
};
