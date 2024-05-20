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
		.filter((i) => i.code !== "")
		.filter((i) => i.rewards.length !== 0)
		.sort((a, b) => {
			const lowPrioritySources = ["HoyoLab Forum", "Eurogamer", "Prydwen"];
			const aIsLowPriority = lowPrioritySources.includes(a.source);
			const bIsLowPriority = lowPrioritySources.includes(b.source);

			if (aIsLowPriority && !bIsLowPriority) {
				return 1;
			}
			if (!aIsLowPriority && bIsLowPriority) {
				return -1;
			}

			return 0;
		})
		.filter((i, index, self) => self.findIndex((t) => t.code === i.code) === index)
		.filter(Boolean);

	debug.info(`Found ${data.length} codes to be checked.`);

	if (codeData.length === 0) {
		logger.warn("No codes found in database. Inserting all codes...");
		await app.Query.collection("codes").insertMany(data.map(i => ({ ...i, date: new Date(), active: false })));
		return [];
	}

	const filteredData = data.filter(i => !codes.has(i.code));
	if (filteredData.length === 0) {
		debug.info("No new codes found.");
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
