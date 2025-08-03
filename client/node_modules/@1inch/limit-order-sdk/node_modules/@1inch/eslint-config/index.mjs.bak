import { fixupConfigRules, fixupPluginRules } from "@eslint/compat";
import typescriptEslint from "@typescript-eslint/eslint-plugin";
import unusedImports from "eslint-plugin-unused-imports";
import _import from "eslint-plugin-import";
import globals from "globals";
import tsParser from "@typescript-eslint/parser";
import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import { FlatCompat } from "@eslint/eslintrc";
import eslintConfigPrettier from "eslint-config-prettier";
import stylistic from '@stylistic/eslint-plugin'

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
    baseDirectory: __dirname,
    recommendedConfig: js.configs.recommended,
    allConfig: js.configs.all
});

export default [{
    ignores: [
        "**/*.mock.ts",
        "**/node_modules",
        "**/.github",
        "**/.idea",
        "**/.eslintrc-tmp.js",
        "**/dist",
        "**/*.json",
        "**/*.yml",
        "**/*.html",
        "**/graph.serviceuser.postgres.repository.service.ts",
        "**/migrations",
    ],
}, ...fixupConfigRules(compat.extends(
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:prettier/recommended",
    "plugin:import/recommended",
    "plugin:import/typescript",
)), {
    plugins: {
        "@typescript-eslint": fixupPluginRules(typescriptEslint),
        "unused-imports": unusedImports,
        import: fixupPluginRules(_import),
        '@stylistic': stylistic
    },

    languageOptions: {
        globals: {
            ...globals.node,
        },

        parser: tsParser,
    },

    settings: {
        "import/resolver": {
            typescript: {},
        },
    },


    rules: {
        "import/namespace": "off",
        "import/default": "off",
        "@typescript-eslint/member-ordering": "error",
        "lines-between-class-members": "error",

        "padding-line-between-statements": ["error", {
            blankLine: "always",
            prev: "*",
            next: "return",
        }, {
            blankLine: "always",
            prev: "*",
            next: "if",
        }, {
            blankLine: "always",
            prev: "if",
            next: "*",
        }, {
            blankLine: "always",
            prev: "for",
            next: "*",
        }, {
            blankLine: "always",
            prev: "*",
            next: "for",
        }],

        "@typescript-eslint/adjacent-overload-signatures": "off",
        "no-unused-vars": "off",
        "no-prototype-builtins": "off",

        "max-len": ["error", {
            code: 120,
            ignoreComments: true,
            ignorePattern: "import\\s.+\\sfrom\\s'.+';?$",
        }],

        "max-depth": ["error", 3],
        "max-lines-per-function": ["error", 255],
        "max-params": ["error", 10],
        "@typescript-eslint/no-explicit-any": "warn",

        "@typescript-eslint/no-unused-vars": ["error", {
            argsIgnorePattern: "^_",
        }],

        "unused-imports/no-unused-imports": "error",
        "unused-imports/no-unused-vars": 0,
        "no-async-promise-executor": 0,
        "no-console": "error",
        "@typescript-eslint/explicit-function-return-type": "error",
        "@typescript-eslint/no-non-null-assertion": "off",

        "import/order": ["error", {
            groups: ["external", "builtin", "internal", "sibling", "parent", "index"],
        }],
        "prettier/prettier": ["error", { "semi": false }],
        '@stylistic/comma-dangle': ['error', 'never'],
    },
}, {
    files: ["src/**/*.test.ts", "src/**/*.integration-test.ts", "src/**/*.spec.ts"],

    rules: {
        "max-lines-per-function": ["error", 1000],

        "max-len": ["error", {
            code: 1130,
        }],
    },
},
    eslintConfigPrettier
];