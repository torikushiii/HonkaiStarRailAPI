const EurogamerResolver = require("./eurogamer-resolver.js");
const Game8Resolver = require("./game8-resolver.js");
const PolygonResolver = require("./polygon-resolver.js");

const fetch = async () => {
	const promises = await Promise.all([
		EurogamerResolver.fetch(),
		Game8Resolver.fetch(),
		PolygonResolver.fetch()
	]);

	return [...new Set(promises.flat())];
};

module.exports = {
	fetch
};
