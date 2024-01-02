exports.fetch = async () => {
	try {
		const debug = await app.Logger("debug:resolver:prydwen");
		const logger = await app.Logger("resolver:prydwen");

		const res = await app.Got({
			url: "https://www.prydwen.gg/star-rail/",
			responseType: "text"
		});

		if (res.statusCode !== 200) {
			logger.info("Failed to fetch data from Prydwen.", {
				statusCode: res.statusCode,
				response: res.body
			});

			return [];
		}

		const codes = [];
		const $ = app.Utils.cheerio(res.body);

		const $codes = $(".codes .box");
		for (let i = 0; i < $codes.length; i++) {
			const $code = $($codes[i]);
			const code = $code.find(".code").text().replace(" NEW!", "");
			const rewards = $code.find(".rewards").text();
			codes.push({
				code: code.trim(),
				rewards: rewards.split(" + "),
				source: "Prydwen"
			});
		}

		if (codes.length === 0) {
			debug.error(res.body);
			logger.error("No codes found.");
			return [];
		}

		debug.info(`Found ${codes.length} codes.`, { codes });

		return codes;
	}
	catch {
		return [];
	}
};
