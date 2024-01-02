exports.fetch = async () => {
	try {
		const debug = await app.Logger("debug:resolver:hsr-fandom");
		const logger = await app.Logger("resolver:hsr-fandom");

		const res = await app.Got({
			url: "https://honkai-star-rail.fandom.com/wiki/Redemption_Code",
			responseType: "text"
		});

		if (res.statusCode !== 200) {
			logger.info("Failed to fetch data from HSR Fandom.", {
				statusCode: res.statusCode,
				response: res.body
			});

			return [];
		}
    
		const $ = app.Utils.cheerio(res.body);
    
		const table = $("#mw-content-text > div.mw-parser-output > table > tbody").toArray().map(i => $(i).text());
		const tableList = table[0].split("\n").filter(Boolean).slice(3);
    
		const rewards = [];
		for (const row of tableList) {
			const cleanText = row.replace(/All|(\[\d+\])/g, "").trim();
			const codeRegex = /HSRGRANDOPEN[0-9]|[A-Z0-9]{12}/;
			const code = cleanText.match(codeRegex)?.[0];
			if (!code) {
				continue;
			}
    
			const reward = cleanText.replace(codeRegex, "").trim().split("Discovered")[0];
			const rewardList = reward.split(/×\d+/g).filter(Boolean).map(i => i.trim());
			const rewardAmount = reward.match(/×\d+/g).map(i => i.replace("×", "").trim());
    
			const joined = [];
			for (let i = 0; i < rewardList.length; i++) {
				joined.push(`${rewardList[i]} x${rewardAmount[i]}`);
			}
    
			rewards.push({
				code,
				rewards: joined,
				source: "star-rail-fandom"
			});
		}

		debug.info(`Found ${rewards.length} codes.`, { rewards });

		return rewards;
	}
	catch {
		return [];
	}
};
