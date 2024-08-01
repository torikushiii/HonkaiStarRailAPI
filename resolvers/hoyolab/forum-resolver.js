exports.fetch = async () => {
	try {
		const debug = await app.Logger("debug:resolver:hoyolab:forum");
		const logger = await app.Logger("resolver:hoyolab:forum");

		const res = await app.Got({
			url: "https://bbs-api-os.hoyolab.com/community/painter/wapi/search",
			searchParams: {
				game_id: 6,
				is_all_game: false,
				keyword: "redemption code",
				page_num: 1,
				page_size: 20
			},
			headers: {
				"x-rpc-app_version": "2.43.0",
				"x-rpc-client_type": 4
			}
		});

		if (res.statusCode !== 200) {
			logger.info("Failed to fetch data from Hoyolab API.", {
				statusCode: res.statusCode,
				response: res.body
			});

			return [];
		}

		const posts = res.body.data.posts;
		if (posts.length === 0) {
			debug.warn(res.body);
			debug.warn("No posts found.");
			return;
		}

		const codeRegex = /[A-Z0-9]{12}/;
		const rewardRegex = /Rewards: (.*)/;
		const unwantedPhrase = "Redemption code is only valid until:";

		const codes = posts.map(i => {
			const code = i.post.content.match(codeRegex);
			if (code) {
				const rewards = i.post.content.match(rewardRegex);
				let rewardsData = rewards?.[1].split(/&|,/).map(i => i.trim()) ?? [];
				rewardsData = rewardsData.map(reward => reward.replace(unwantedPhrase, "").trim());
				return {
					code: code[0],
					rewards: rewardsData.filter(reward => reward.length > 0),
					source: "HoyoLab Forum"
				};
			}
		}).filter(Boolean);

		debug.info(`Found ${codes.length} codes.`, { codes });

		return [...new Set(codes)];
	}
	catch {
		return [];
	}
};
