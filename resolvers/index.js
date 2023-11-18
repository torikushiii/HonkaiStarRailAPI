const HoyoLab = require("./hoyolab");
const Prydwen = require("./prydwen");
const Defaults = require("./defaults");

const fetchAll = async () => {
	const debug = await app.Logger("debug:resolver");
	const logger = await app.Logger("resolver");

	const promises = await Promise.all([
		HoyoLab.fetch(),
		Prydwen.fetch(),

		Defaults.fetch()
	]);

	const data = promises
		.flat()
		.filter((code) => code)
		.filter((code, index, self) => self.findIndex((i) => i.code === code.code) === index);

	debug.info(`Found ${data.length} codes to be checked.`);

	const codeData = await app.Query.collection("codes").find({}).toArray();
	if (codeData.length === 0) {
		logger.warn("No codes found in database. Inserting all available codes...");
		for (const code of data) {
			code.date = new Date();
			code.active = false;
		}

		await app.Query.collection("codes").insertMany(data);

		logger.info(`Inserted ${data.length} codes.`);
		return;
	}

	const codes = codeData.map((code) => code.code);

	const filteredData = data.filter((code) => !codes.includes(code.code));
	if (filteredData.length === 0) {
		debug.info("No new codes found.");
		return [];
	}

	logger.info(`Found ${filteredData.length} new codes. Inserting...`);
	logger.info(filteredData);

	for (const code of filteredData) {
		code.date = new Date();
		code.active = true;
	}

	await app.Query.collection("codes").insertMany(filteredData);

	return filteredData;
};

module.exports = {
	fetchAll
};
