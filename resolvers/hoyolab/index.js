const ApiResolver = require("./api-resolver.js");
const ForumResolver = require("./forum-resolver.js");

const fetch = async () => {
	const promises = await Promise.all([
		ApiResolver.fetch(),
		ForumResolver.fetch()
	]);

	return [...new Set(promises.flat())];
};

module.exports = {
	fetch
};
