const config = require("../config.js");

const checkAndRedeem = async (codeList) => {
	const debug = await app.Logger("debug:redeemer");
	const logger = await app.Logger("redeemer");

	if (!Array.isArray(codeList)) {
		throw new app.Error({
			message: "Expected an array of codes.",
			args: {
				codeList,
				typeof: typeof codeList
			}
		});
	}

	if (codeList.length === 0) {
		debug.info("No codes to check.");
		return [];
	}

	const account = config.accountCredential;
	if (!account) {
		logger.warn("No account credentials provided. Skipping code redemption.");
		return;
	}

	for (const data of codeList) {
		const res = await app.Got({
			url: "https://sg-hkrpg-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey",
			searchParams: {
				cdkey: data.code,
				game_biz: "hkrpg_global",
				lang: "en",
				region: account.region,
				t: Date.now(),
				uid: account.uid
			},
			headers: {
				cookie: account.cookie
			}
		});

		if (res.statusCode !== 200) {
			logger.error(`Failed to redeem code ${data.code}.`, {
				response: res.body,
				statusCode: res.statusCode
			});

			await new Promise((resolve) => setTimeout(resolve, 10000));
			continue;
		}

		if (res.body.retcode === -1071) {
			await new Promise((resolve) => setTimeout(resolve, 10000));
			throw new app.Error({
				message: "Invalid account credentials.",
				args: {
					response: res.body
				}
			});
		}
		if (res.body.retcode === -2017) {
			await new Promise((resolve) => setTimeout(resolve, 10000));
			logger.warn(`Code ${data.code} is already redeemed. Skipping...`);
			continue;
		}
		if (res.body.retcode !== 0) {
			await new Promise((resolve) => setTimeout(resolve, 10000));
			throw new app.Error({
				message: "Unknown error while redeeming code.",
				args: {
					code: data.code,
					response: res.body
				}
			});
		}

		logger.info(`Successfully redeemed code ${data.code}.`);
		
		await sendNotification(data);
		await new Promise((resolve) => setTimeout(resolve, 10000));
	}

	return true;
};

const validateRedeemCodes = async () => {
	const debug = await app.Logger("debug:redeemer");
	const logger = await app.Logger("redeemer");

	const codeData = await app.Query.collection("codes").find({ active: true, code: { $ne: "STARRAILGIFT" } }).toArray();
	if (codeData.length === 0) {
		logger.info("No codes found in database.");
		return;
	}

	logger.info(`Found ${codeData.length} codes to be checked.`);
	debug.info(codeData);

	const account = config.accountCredential;
	if (!account) {
		logger.warn("No account credentials provided. Skipping code redemption.");
		return;
	}

	const activeCodes = [];
	const inactiveCodes = [];
	for (const data of codeData) {
		const res = await app.Got({
			url: "https://sg-hkrpg-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey",
			searchParams: {
				cdkey: data.code,
				game_biz: "hkrpg_global",
				lang: "en",
				region: account.region,
				t: Date.now(),
				uid: account.uid
			},
			headers: {
				cookie: account.cookie
			}
		});

		if (res.statusCode !== 200) {
			logger.error(`Failed to redeem code ${data.code}.`, {
				response: res.body,
				statusCode: res.statusCode
			});

			continue;
		}

		if (res.body.retcode === -1071) {
			throw new app.Error({
				message: "Invalid account credentials.",
				args: {
					response: res.body
				}
			});
		}

		console.log(res.statusCode, res.body);

		// -2017: "Already redeemed"
		// -2001: "Expired"
		// -2003: "Invalid code"
		// -2016: "Cooldown"
		if (res.body.retcode === -2017) {
			// skip this code because it's still active
			activeCodes.push(data.code);
			await new Promise((resolve) => setTimeout(resolve, 15000));
			continue;
		}
		else if (res.body.retcode === -2001 || res.body.retcode === -2003) {
			logger.warn(`Code ${data.code} is expired.`, {
				response: res.body
			});

			logger.warn(`Marking code ${data.code} as inactive...`);

			const ops = await app.Query.collection("codes").updateOne({ code: data.code }, {
				$set: {
					active: false
				}
			});

			if (!ops.acknowledged) {
				throw new app.Error({
					message: "Failed to mark code as inactive.",
					args: {
						code: data.code,
						ops
					}
				});
			}

			inactiveCodes.push(data.code);
			await new Promise((resolve) => setTimeout(resolve, 15000));
			continue;
		}
		else if (res.body.retcode === -2016) {
			logger.warn(`Code ${data.code} is in cooldown.`, {
				response: res.body
			});

			await new Promise((resolve) => setTimeout(resolve, 15000));
			continue;
		}
	}

	return {
		activeCodes,
		inactiveCodes
	};
};

const sendNotification = async (codeData) => {
	const { DISCORD_WEBHOOK } = config;

	const res = await app.Got({
		url: DISCORD_WEBHOOK,
		method: "POST",
		responseType: "json",
		searchParams: {
			wait: true
		},
		json: {
			embeds: [
				{
					color: 0xBB0BB5,
					title: "Honkai: Star Rail New Code",
					description: `Code: ${codeData.code}`
					+ `\n Rewards:\n${codeData.rewards.join(", ")}`
					+ `\n\n Claim here:\nhttps://hsr.hoyoverse.com/gift?code=${codeData.code}`,
					timestamp: new Date(),
					footer: {
						text: "Honkai: Star Rail New Code"
					}
				}
			],
			username: "Honkai: Star Rail",
			avatar_url: "https://i.imgur.com/o0hyhmw.png"
		}
	});

	if (res.statusCode !== 200) {
		throw new app.Error({
			message: "Failed to send message to Discord",
			args: {
				statusCode: res.statusCode,
				statusMessage: res.statusMessage,
				body: res.body
			}
		});
	}

	return true;
};

module.exports = {
	checkAndRedeem,
	validateRedeemCodes
};
