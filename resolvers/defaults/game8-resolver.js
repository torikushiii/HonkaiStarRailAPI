exports.fetch = async () => {
	try {
		const debug = await app.Logger("debug:resolver:game8");
		const logger = await app.Logger("resolver:game8");

		const res = await app.Got({
			url: "https://game8.co/games/Honkai-Star-Rail/archives/410296",
			responseType: "text"
		});

		if (res.statusCode !== 200) {
			logger.info("Failed to fetch data from Game8.", {
				statusCode: res.statusCode,
				response: res.body
			});

			return [];
		}

		const $ = app.Utils.cheerio(res.body);

		const codes = [];
		const $codes = $(".a-listItem");
		if ($codes.length === 0) {
			debug.error(res.body);
			logger.error("No codes found.");
		}

		for (let i = 0; i < $codes.length; i++) {
			const $code = $($codes[i]);
			const code = $code.find(".a-bold").text();
			// eslint-disable-next-line newline-per-chained-call
			const rewards = $code.text().replace(code, "").trim().split(", ");

			codes.push({
				code: code.trim(),
				rewards: rewards.map((reward) => reward.replace(/\(|\)/g, "").trim()),
				source: "game8"
			});
		}

		debug.info(`Found ${codes.length} codes.`, { codes });

		return codes;
	}
	catch {
		return [];
	}
};
