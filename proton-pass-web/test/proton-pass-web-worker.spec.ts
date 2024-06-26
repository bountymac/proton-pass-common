import { describe, expect, test } from "bun:test";
import * as packageJSON from "./pkg/package.json";

import {
    generate_passkey,
    generate_passphrase,
    library_version,
    random_words,
} from "./pkg/worker";

describe("ProtonPassWeb WASM", () => {
    test("Library version", () => {
        expect(library_version()).toEqual(packageJSON.version);
    });

    test("Should create passphrase", () => {
        const words = random_words(3);
        const passphrase = generate_passphrase(words, {
            separator: "Hyphens",
            capitalise: true,
            include_numbers: true,
            count: 3,
        });

        expect(passphrase).not.toBeUndefined();
    });

    test("Can generate passkey", async () => {
        const input = {
            attestation: "none",
            authenticatorSelection: {
                residentKey: "preferred",
                userVerification: "preferred",
            },
            challenge:
                "D-5y7y_E4V8NQBJrFnnhd7NCvRGhO5sBGwzfh23y8D4a_hSMyRRuTAp0hmSm6_eimM71XoYF84VUiY8e9kqavA",
            excludeCredentials: [],
            extensions: { credProps: true },
            pubKeyCredParams: [
                { alg: -7, type: "public-key" },
                { alg: -257, type: "public-key" },
            ],
            rp: { id: "webauthn.io", name: "webauthn.io" },
            user: { displayName: "uyguyhj", id: "ZFhsbmRYbG9hZw", name: "uyguyhj" },
        };
        const inputString = JSON.stringify(input);
        const response = await generate_passkey("https://webauthn.io", inputString);
        expect(response.credential).not.toBeEmpty();
        expect(response.passkey).not.toBeEmpty();
        expect(response.user_id).not.toBeEmpty();
        expect(response.key_id).not.toBeEmpty();
    });
});
