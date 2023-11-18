exports.fetch = async () => {
	try {
		const debug = await app.Logger("debug:resolver:eurogamer");
		const logger = await app.Logger("resolver:eurogamer");

		const res = await app.Got({
			url: "https://www.eurogamer.net/honkai-star-rail-codes-livestream-active-working-how-to-redeem-9321",
			responseType: "text"
		});

		if (res.statusCode !== 200) {
			logger.info("Failed to fetch data from Eurogamer.", {
				statusCode: res.statusCode,
				response: res.body
			});
		}

		const $ = app.Utils.cheerio(res.body);
		const codes = $("ul li strong").toArray().map(i => $(i).text());
		if (codes.length === 0) {
			debug.error(res.body);
			logger.error("No codes found.");
		}

		const table = $("table").toArray().map(i => $(i).text());
		const tableList = table[0].split("\n").filter(Boolean).slice(3);

		let reward = {};
		const rewards = [];
		for (const [index, i] of tableList.entries()) {
			if (index % 3 === 0) {
				reward.code = i;
			}
			else if (index % 3 === 1) {
				reward.rewards = i.split(" and ");
			}
			else if (index % 3 === 2) {
				rewards.push({
					code: reward.code,
					rewards: reward.rewards,
					source: "Eurogamer"
				});
				
				reward = {};
			}
		}

		debug.info(`Found ${codes.length} codes.`, { rewards });

		return rewards;
	}
	catch {
		return [];
	}
};
