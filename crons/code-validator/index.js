const definition = {
	name: "code-validator",
	expression: "*/30 * * * *",
	code: (async function codeValidator () {
		const redeemer = require("../../redeemer");

		const res = await redeemer.validateRedeemCodes();
		if (res.activeCodes.length === 0 && res.inactiveCodes.length === 0) {
			return;
		}

		const log = await app.Logger("code-validator");
		if (res.activeCodes.length > 0) {
			log.info(`Processed ${res.activeCodes.length} active codes.`);
		}
		if (res.inactiveCodes.length > 0) {
			log.info(`Processed ${res.inactiveCodes.length} inactive codes.`);
		}
	})
};

module.exports = definition;
