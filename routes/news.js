module.exports = function (fastify, opts, done) {
	const Router = fastify;

	Router.get("/", async (req, res) => res.send({
		message: "Not implemented"
	}));

	done();
};
