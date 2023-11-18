const Cron = require("./classes/cron.js");

const CustomError = require("./object/error.js");

const Utils = require("./singletons/utils.js");
const GotModule = require("./singletons/got.js");
const Logger = require("./singletons/logger.js");
const DatabaseManager = require("./singletons/query.js");

require("./db-access.js");

(async () => {
	globalThis.app = {
		Cron,
		Logger
	};

	const logger = await app.Logger("init:server");
	const debug = await app.Logger("debug:init:server");

	logger.info("Initializing services...");

	const Query = new DatabaseManager({
		mongoIp: process.env.MONGO_IP,
		mongoPort: process.env.MONGO_PORT,
		mongoDbName: process.env.MONGO_DB_NAME
	});

	const resolvers = require("./resolvers");
	const redeemer = require("./redeemer");
	const crons = [
		new app.Cron({
			name: "fetch-data",
			expression: "*/10 * * * *",
			code: (async function initializer () {
				const codeList = await resolvers.fetchAll();
				await redeemer.checkAndRedeem(codeList);
			})
		}),
		new app.Cron({
			name: "code-validation",
			expression: "*/30 * * * *",
			code: (async function initializer () {
				const res = await redeemer.validateRedeemCodes();
				if (res.activeCodes.length === 0 && res.inactiveCodes.length === 0) {
					return;
				}

				const log = await app.Logger("code-validation");
				if (res.activeCodes.length > 0) {
					log.info(`Processed ${res.activeCodes.length} active codes.`);
				}
				if (res.inactiveCodes.length > 0) {
					log.info(`Processed ${res.inactiveCodes.length} inactive codes.`);
				}
			})
		})
	];

	for (const cron of crons) {
		cron.start();
	}

	globalThis.app = {
		...app,
		Error: CustomError,
        
		Utils: new Utils(),
		Got: await GotModule,
		Query: Query.client()
	};

	logger.info("Initializing server...");

	const config = require("./config.js");
	const fastify = require("fastify")({
		trustProxy: true,
		disableRequestLogging: true
	});

	fastify.register(require("@fastify/sensible"));
	fastify.setErrorHandler(async (error, request, reply) => {
		const statusCode = error.statusCode ?? reply.statusCode ?? 500;

		const response = {
			statusCode,
			error: error.name,
			message: error.message
		};

		if (statusCode >= 500) {
			try {
				const requestId = await app.Query.collection("errors").countDocuments() + 1;
				await app.Query.collection("errors").insertOne({
					id: requestId,
					error: error.name,
					name: error.name,
					message: error.message,
					stack: error.stack
				});
			}
			catch (e) {
				console.error("Failed to log error to database.", e);
			}
		}

		return reply.status(statusCode).send(response);
	});

	fastify.get("/starrail/", async (request, reply) => {
		reply.send({
			statusCode: 200,
			routes: [
				"/starrail/code",
				"/starrail/news"
			]
		});
	});

	const subroutes = [
		"code",
		"news"
	];

	for (const subroute of subroutes) {
		fastify.register(require(`./routes/${subroute}.js`), { prefix: `starrail/${subroute}` });
	}

	fastify.get("*", async (request, reply) => {
		reply.notFound();
	});

	fastify.get("/starrail", async (request, reply) => {
		reply.redirect("/starrail/");
	});

	logger.info("All services initialized.");

	fastify.listen({ port: config.port, host: config.host }, (error, address) => {
		if (error) {
			console.error(error);
			process.exit(1);
		}

		logger.info(`Server listening on ${address}`);
	});

	process.on("unhandledRejection", async (error) => {
		if (!(error instanceof Error)) {
			return;
		}

		if (error.name === "RequestError") {
			return;
		}

		debug.error("Error received.", error);

		try {
			await app.Query.collection("errors").insertOne({
				id: await app.Query.collection("errors").countDocuments() + 1,
				error: error.name,
				name: error.name,
				message: error.message,
				stack: error.stack
			});
		}
		catch (e) {
			debug.error("Error occurred while logging error to database.", e);
			logger.error("Failed to log error to database.", error);
		}
	});
})();
