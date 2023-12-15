const definition = {
	name: "code-redemption-scraper",
	expression: "* * * * *",
	code: (async function fetchCodeRedemption () {
		const resolvers = require("../../resolvers");
		const redeemer = require("../../redeemer");

		const codeList = await resolvers.fetchAll();
		await redeemer.checkAndRedeem(codeList);
	})
};

module.exports = definition;
