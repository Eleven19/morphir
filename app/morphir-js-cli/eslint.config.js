import pluginJs from "@eslint/js";
import parser from "@typescript-eslint/parser";
import globals from "globals";
import tseslint from "typescript-eslint";

export default [
    { languageOptions: { globals: globals.browser } },
    pluginJs.configs.recommended,
    ...tseslint.configs.recommended,
    {
      languageOptions: {
        parser,
        parserOptions: {
          ecmaVersion: 2020,
          sourceType: "module",
        },
      },
      // todo add "plugins": ["import", "simple-import-sort"],
      rules: {
        "no-unused-vars": "off",
        "@typescript-eslint/no-unused-vars": [
          "error",
          { argsIgnorePattern: "^_" },
        ],
        // "import/first": "error",
        // "import/no-cycle": "error",
        // "import/newline-after-import": "error",
        // "import/no-duplicates": "error",
        "@typescript-eslint/consistent-type-imports": "error",
      },
    },
  ];