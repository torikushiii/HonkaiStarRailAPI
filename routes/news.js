module.exports = function (fastify, opts, done) {
	const Router = fastify;

	Router.get("/", async (req, res) => res.send({
		endpoints: [
			"news/events",
			"news/notices",
			"news/info"
		]
	}));

	Router.get("/events", async (req, res) => {
		const lang = req.query.lang || "en";
		const langCode = app.Utils.languageCodeConverter(lang);

		const eventData = await app.Query.collection("news")
			.find({ type: "event", lang: langCode })
			.project({ type: 0, lang: 0, _id: 0 })
			.toArray();
		
		if (eventData.length === 0) {
			return res.send([]);
		}

		return res.send(eventData);
	});

	Router.get("/notices", async (req, res) => {
		const lang = req.query.lang || "en";
		const langCode = app.Utils.languageCodeConverter(lang);

		const noticeData = await app.Query.collection("news")
			.find({ type: "notice", lang: langCode })
			.project({ type: 0, lang: 0, _id: 0 })
			.toArray();
		
		if (noticeData.length === 0) {
			return res.send([]);
		}

		return res.send(noticeData);
	});

	Router.get("/info", async (req, res) => {
		const lang = req.query.lang || "en";
		const langCode = app.Utils.languageCodeConverter(lang);

		const infoData = await app.Query.collection("news")
			.find({ type: "info", lang: langCode })
			.project({ type: 0, lang: 0, _id: 0 })
			.toArray();
		
		if (infoData.length === 0) {
			return res.send([]);
		}

		return res.send(infoData);
	});

	done();
};
