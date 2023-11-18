exports.fetch = async () => {
	const debug = await app.Logger("debug:resolver:hoyolab:api");
	const logger = await app.Logger("resolver:hoyolab:api");

	const res = await app.Got({
		url: "https://bbs-api-os.hoyolab.com/community/painter/wapi/circle/channel/guide/material",
		searchParams: {
			game_id: 6
		},
		headers: {
			"x-rpc-app_version": "2.42.0",
			"x-rpc-client_type": 4
		}
	});

	if (res.statusCode !== 200) {
		logger.info("Failed to fetch data from Hoyolab API.", {
			statusCode: res.statusCode,
			response: res.body
		});
	}

	const exchangeGroup = res.body.data.modules.find(i => i.exchange_group !== null);
	if (!exchangeGroup) {
		debug.warn(res.body);
		debug.warn("No exchange group found.");
		return [];
	}

	const pictureHash = [
		{
			hash: "77cb5426637574ba524ac458fa963da0_6409817950389238658",
			name: "Stellar Jade"
		},
		{
			hash: "7cb0e487e051f177d3f41de8d4bbc521_2556290033227986328",
			name: "Refined Aether"
		},
		{
			hash: "508229a94e4fa459651f64c1cd02687a_6307505132287490837",
			name: "Traveler's Guide"
		},
		{
			hash: "0b12bdf76fa4abc6b4d1fdfc0fb4d6f5_4521150989210768295",
			name: "Credit"
		}
	];

	// finish this when there's redemption at HoyoLab
	return [];
};
