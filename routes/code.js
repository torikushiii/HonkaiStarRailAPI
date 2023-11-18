module.exports = function (fastify, opts, done) {
	const Router = fastify;

	Router.get("/", async (req, res) => {
		const codes = await app.Query.collection("codes").find({}).toArray();
		if (codes.length === 0) {
			return res.send({
				active: [],
				inactive: []
			});
		}

		const active = codes.filter((code) => code.active).map(i => ({
			code: i.code,
			rewards: i.rewards
		}));

		const inactive = codes.filter((code) => !code.active).map(i => ({
			code: i.code,
			rewards: i.rewards
		}));

		return res.send({
			active,
			inactive
		});
	});

	done();
};
