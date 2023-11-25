const definition = {
	name: "news-update",
	expression: "*/5 * * * *",
	code: (async function hoyolabNewsUpdate () {
		const debug = await app.Logger("news-update");

		const newsUpdater = require("../../resolvers/news");
		try {
			await newsUpdater.fetch();
		}
		catch (e) {
			debug.error("Failed to fetch news.", e);
		}
	})
};

module.exports = definition;
