const events = (data) => {
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
			type: "event"
		});
	}

	return result;
};

const notices = (data) => {
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
			type: "notice"
		});
	}

	return result;
};

const info = (data) => {
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
			type: "info"
		});
	}

	return result;
};

module.exports = {
	events,
	notices,
	info
};
