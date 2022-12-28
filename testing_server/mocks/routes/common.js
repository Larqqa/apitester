// Use this file only as a guide for first steps using middleware variants. You can delete it when you have understood the concepts.
// For a detailed explanation about using middlewares, visit:
// https://mocks-server.org/docs/usage/variants/middlewares

module.exports = [
  {
    id: "debug", //route id
    url: "*", // url in express format
    method: ["GET", "POST", "PUT", "PATCH", "DELETE"], // HTTP methods
    variants: [
      {
        id: "enabled", // variant id
        type: "middleware", // variant handler id
        options: {
          middleware: (_req, res) => {
            console.table(_req.headers);
            console.log("Body:", _req.body);
            console.log("Query:", _req.query);

            res.status(200);
            if (_req.body) {
              res.send({ result: _req.body });
            } else {
              res.send({ result: true});
            }
          },
        },
      },
      {
        id: "disabled", // variant id
        disabled: true,
      },
    ],
  },
];
