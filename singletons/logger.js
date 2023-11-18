/* eslint-disable no-fallthrough */
let LOG_LEVEL = +process.env.LOG_LEVEL;
if (!LOG_LEVEL) {
	LOG_LEVEL = 2;
}
else if (LOG_LEVEL < 0) {
	LOG_LEVEL = 0;
}
else if (LOG_LEVEL > 3) {
	LOG_LEVEL = 3;
}

const debug = async (namespace) => {
	const DebugModule = await import("debug");
	const Debug = DebugModule.default;

	const logger = Debug(namespace);

	const log = logger.extend("[ LOG ]:");
	log.log = console.log.bind(console);

	const info = logger.extend("[ INFO ]:");
	info.log = console.info.bind(console);
	info.color = 4;

	const warn = logger.extend("[ WARN ]:");
	warn.log = console.warn.bind(console);
	warn.color = 9;

	const error = logger.extend("[ ERROR ]:");
	error.log = console.error.bind(console);
	error.color = 196;

	switch (LOG_LEVEL) {
		case 0:
			warn.enabled = false;
		case 1:
			info.enabled = false;
		case 2:
			log.enabled = false;
	}

	return {
		extend: extend.bind(logger),
		log,
		info,
		warn,
		error
	};
};

const extend = (namespace) => debug(`${this.namespace}:${namespace}`);

module.exports = debug;
