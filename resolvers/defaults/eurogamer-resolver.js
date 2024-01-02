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

			return [];
		}

		const $ = app.Utils.cheerio(res.body);
		const activeCodes = [];

		const list = $("#content_above > div.page_content > article > div > div > ul:nth-child(14) > li");
		for (let i = 0; i < list.length; i++) {
			const el = list[i];
			const text = $(el).text().trim();
			const code = text.split(":")[0].trim();
			const reward = text.split(":")[1].trim().split(", ").map(i => i.replace(/\(new!\)|and/g, "").trim());

			activeCodes.push({
				code,
				rewards: reward,
				source: "Eurogamer"
			});
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
				reward.rewards = i.split(" and ").flatMap(i => i.split(", "));
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

		const codes = [...activeCodes, ...rewards];

		debug.info(`Found ${codes.length} codes.`, { codes });

		return codes;
	}
	catch {
		return [];
	}
};
