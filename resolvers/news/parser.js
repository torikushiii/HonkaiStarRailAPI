const events = (data, lang) => {
	if (data.length === 0) {
		return [];
	}

	const result = [];
	for (const item of data) {
		result.push({
			id: item.id,
			title: item.name,
			description: item.desc,
			createdAt: +item.create_at,
			startAt: +item.start,
			endAt: +item.end,
			banner: item.banner_url,
			url: `https://www.hoyolab.com/article/${item.id}`,
			type: "event",
			lang
		});
	}

	return result;
};

const notices = (data, lang) => {
	if (data.length === 0) {
		return [];
	}

	const parseImage = (images) => {
		if (images.length === 0) {
			return null;
		}

		const image = [];
		for (const item of images) {
			image.push(item.url);
		}

		return image;
	};

	const result = [];
	for (const item of data) {
		result.push({
			id: item.post.post_id,
			title: item.post.subject,
			description: item.post.content,
			createdAt: +item.post.created_at,
			banner: parseImage(item.image_list),
			url: `https://www.hoyolab.com/article/${item.post.post_id}`,
			type: "notice",
			lang
		});
	}

	return result;
};

const info = (data, lang) => {
	if (data.length === 0) {
		return [];
	}

	const parseImage = (images) => {
		if (images.length === 0) {
			return null;
		}

		const image = [];
		for (const item of images) {
			image.push(item.url);
		}

		return image;
	};

	const result = [];
	for (const item of data) {
		result.push({
			id: item.post.post_id,
			title: item.post.subject,
			description: item.post.content,
			createdAt: +item.post.created_at,
			banner: parseImage(item.image_list),
			url: `https://www.hoyolab.com/article/${item.post.post_id}`,
			type: "info",
			lang
		});
	}

	return result;
};

module.exports = {
	events,
	notices,
	info
};
