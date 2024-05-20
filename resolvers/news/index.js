const parser = require("./parser.js");

const supportedLanguages = [
	"en-us",
	"zh-cn",
	"zh-tw",
	"de-de",
	"es-es",
	"fr-fr",
	"id-id",
	"it-it",
	"ja-jp",
	"ko-kr",
	"pt-pt",
	"ru-ru",
	"th-th",
	"tr-tr",
	"vi-vn"
];

exports.fetch = async () => {
	for (const lang of supportedLanguages) {
		const eventsRes = app.Got({
			url: "https://bbs-api-os.hoyolab.com/community/community_contribution/wapi/event/list",
			responseType: "json",
			searchParams: {
				page_size: 15,
				size: 15,
				gids: 6
			},
			headers: {
				"x-rpc-app_version": "2.42.0",
				"x-rpc-client_type": 4,
				"x-rpc-Language": lang
			}
		});
	
		const noticesRes = app.Got({
			url: "https://bbs-api-os.hoyolab.com/community/post/wapi/getNewsList",
			responseType: "json",
			searchParams: {
				gids: 6,
				page_size: 15,
				type: 1
			},
			headers: {
				"x-rpc-app_version": "2.42.0",
				"x-rpc-client_type": 4,
				"x-rpc-Language": lang
			}
		});
	
		const infoRes = app.Got({
			url: "https://bbs-api-os.hoyolab.com/community/post/wapi/getNewsList?gids=6&page_size=15&type=3",
			responseType: "json",
			searchParams: {
				gids: 6,
				page_size: 15,
				type: 3
			},
			headers: {
				"x-rpc-app_version": "2.42.0",
				"x-rpc-client_type": 4,
				"x-rpc-Language": lang
			}
		});
	
		const [events, notices, info] = await Promise.allSettled([eventsRes, noticesRes, infoRes]);
		if (events.status === "rejected" || notices.status === "rejected" || info.status === "rejected") {
			throw new app.Error({
				message: "Failed to fetch Hoyolab news.",
				args: {
					events: events.reason,
					notices: notices.reason,
					info: info.reason
				}
			});
		}
	
		const eventsData = parser.events(events.value.body.data.list, lang);
		const noticesData = parser.notices(notices.value.body.data.list, lang);
		const infoData = parser.info(info.value.body.data.list, lang);

		const eventsOps = eventsData.map((event) => ({
			updateOne: {
				filter: { id: event.id, lang },
				update: { $set: event },
				upsert: true
			}
		}));
	
		const noticesOps = noticesData.map((notice) => ({
			updateOne: {
				filter: { id: notice.id, lang },
				update: { $set: notice },
				upsert: true
			}
		}));
	
		const infoOps = infoData.map((info) => ({
			updateOne: {
				filter: { id: info.id, lang },
				update: { $set: info },
				upsert: true
			}
		}));
	
		const ops = await app.Query.collection("news").bulkWrite([
			...eventsOps,
			...noticesOps,
			...infoOps
		]);
	
		if (ops.upsertedCount === 0) {
			return;
		}
	
		const logger = await app.Logger("news-updater");
		logger.info(`Inserted ${ops.upsertedCount} ${lang} new news.`);
	}
};
