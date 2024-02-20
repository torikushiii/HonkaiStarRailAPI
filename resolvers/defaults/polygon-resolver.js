exports.fetch = async () => {
	try {
		const debug = await app.Logger("debug:resolver:polygon");
		const logger = await app.Logger("resolver:polygon");

		const res = await app.Got({
			url: "https://www.polygon.com/honkai-star-rail-guides/23699079/code-redeem-redemption-gift-stellar-jade",
			responseType: "text"
		});

		if (res.statusCode !== 200) {
			logger.info("Failed to fetch data from Polygon.", {
				statusCode: res.statusCode,
				response: res.body
			});

			return [];
		}

		const $ = app.Utils.cheerio(res.body);

		const codes = $("ul li").toArray().map(i => $(i).text());
		if (codes.length === 0) {
			debug.error(res.body);
			logger.error("No codes found.");
		}

		const rewards = [];
		for (const code of codes) {
			const match = code.match(/(.*?)\((.*?)\)/);
			if (!match) {
				continue;
			}

			const [, code_, rewards_] = match;
			const rewards__ = rewards_
				.split(", ")
				.replace(/ and /g, ",");

			rewards.push({
				code: code_.trim(),
				rewards: rewards__.map((reward) => reward.replace(/—.*/g, "").trim()),
				source: "Polygon"
			});
		}

		debug.info(`Found ${codes.length} codes.`, { rewards });

		return rewards;
	}
	catch {
		return [];
	}
};
