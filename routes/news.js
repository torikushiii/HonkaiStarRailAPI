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
		const eventData = await app.Query.collection("news")
			.find({ type: "event" })
			.project({ type: 0, _id: 0 })
			.toArray();
		
		if (eventData.length === 0) {
			return res.send([]);
		}

		return res.send(eventData);
	});

	Router.get("/notices", async (req, res) => {
		const noticeData = await app.Query.collection("news")
			.find({ type: "notice" })
			.project({ type: 0, _id: 0 })
			.toArray();
		
		if (noticeData.length === 0) {
			return res.send([]);
		}

		return res.send(noticeData);
	});

	Router.get("/info", async (req, res) => {
		const infoData = await app.Query.collection("news")
			.find({ type: "info" })
			.project({ type: 0, _id: 0 })
			.toArray();
		
		if (infoData.length === 0) {
			return res.send([]);
		}

		return res.send(infoData);
	});

	done();
};
