const { load } = require("cheerio");

module.exports = class UtilsSingleton {
	/**
	 * @inheritdoc
	 * @returns {UtilsSingleton}
	 */
	static singleton () {
		if (!UtilsSingleton.module) {
			UtilsSingleton.module = new UtilsSingleton();
		}

		return UtilsSingleton.module;
	}

	cheerio (html) {
		return load(html);
	}

	async localizedMaterials (codes, lang) {
		const data = [];
		for (const code of codes) {
			const materials = code.rewards
				.map(i => i.replace(/x/g, "")
					.match(/[a-zA-Z\s']+/g)
					.join("")
					.trim()
				);

			const stuff = [];
			for (const material of materials) {
				const materialData = await app.Query.collection("materials").find({
					lang: "en",
					$text: {
						$search: material
					}
				}).toArray();

				if (materialData.length === 0) {
					// If the material is not found in the database, we'll just push the original material.
					stuff.push(material);
					continue;
				}

				const localizedMaterial = await this.getMaterialById(materialData[0].entry_page_id, lang);
				if (!localizedMaterial) {
					stuff.push(material);
					continue;
				}

				stuff.push(localizedMaterial.name);
			}

			data.push({
				code: code.code,
				materials: stuff,
				active: code.active
			});
		}

		return data;
	}

	async getMaterialById (id, lang = "en") {
		const material = await app.Query.collection("materials").findOne({
			entry_page_id: id,
			lang
		});

		if (!material) {
			return null;
		}

		return material;
	}

	languageCodeConverter (lang) {
		switch (lang) {
			case "en":
				return "en-us";
			case "cn":
				return "zh-cn";
			case "tw":
				return "zh-tw";
			case "de":
				return "de-de";
			case "es":
				return "es-es";
			case "fr":
				return "fr-fr";
			case "id":
				return "id-id";
			case "it":
				return "it-it";
			case "ja":
			case "jp":
				return "ja-jp";
			case "ko":
			case "kr":
				return "ko-kr";
			case "pt":
				return "pt-pt";
			case "ru":
				return "ru-ru";
			case "th":
				return "th-th";
			case "tr":
				return "tr-tr";
			case "vi":
			case "vn":
				return "vi-vn";
			default:
				return "en-us";
		}
	}
};
