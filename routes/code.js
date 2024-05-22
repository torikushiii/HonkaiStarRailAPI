module.exports = function (fastify, opts, done) {
	const Router = fastify;

	Router.get("/", async (req, res) => {
		let lang = req.query.lang || "en";

		const codes = await app.Query.collection("codes").find({}).toArray();
		if (codes.length === 0) {
			return res.send({
				active: [],
				inactive: []
			});
		}

		if (lang !== "en") {
			if (lang === "ja") {
				lang = "jp";
			}
			if (lang === "ko") {
				lang = "kr";
			}

			const rewards = await app.Utils.localizedMaterials(codes, lang);

			const active = rewards.filter((code) => code.active).map(i => ({
				code: i.code,
				rewards: i.materials
			}));

			const inactive = rewards.filter((code) => !code.active).map(i => ({
				code: i.code,
				rewards: i.materials
			}));

			return res.send({
				active,
				inactive
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
