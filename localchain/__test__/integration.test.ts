import {AccountType, MainchainClient, NotaryClient} from "../index";
import TestMainchain from "./TestMainchain";
import TestNotary from "./TestNotary";
import {getClient, Keyring} from "@ulixee/mainchain";
import {describeIntegration, teardown} from "./testHelpers";


afterAll(teardown);

describeIntegration("Integration tests", () => {
    it('can start a mainchain', async () => {
        const mainchain = new TestMainchain();
        const mainchainUrl = await mainchain.launch();
        const url = new URL(mainchainUrl);
        expect(url.protocol).toEqual("ws:");
        expect(url.port).toBeTruthy();
    });


    it('can get a ticker', async () => {
        const mainchain = new TestMainchain();
        const mainchainUrl = await mainchain.launch();

        const client = await MainchainClient.connect(mainchainUrl, 2000);

        await expect(client.getTicker()).resolves.toBeTruthy();
    });

    it('can start a notary', async () => {
        const mainchain = new TestMainchain();
        const mainchainUrl = await mainchain.launch();
        const notary = new TestNotary();
        const notaryUrl = await notary.start(mainchainUrl);
        expect(notaryUrl).toBeTruthy();
        const url = new URL(notaryUrl);
        expect(url.protocol).toEqual("ws:");
        expect(url.port).toBeTruthy();

        const addressKeyring = new Keyring({type: 'sr25519'});
        const bob = addressKeyring.createFromUri('//Bob', {type: 'ed25519'});
        const client = await NotaryClient.connect(1, Buffer.from(bob.publicKey), notaryUrl, false);
        const metadata = await client.metadata;
        expect(metadata.finalizedNotebookNumber).toBeGreaterThanOrEqual(0);

        await expect(client.getBalanceTip(bob.address, AccountType.Deposit)).rejects.toThrow();

    });

    it('can register a notary', async () => {
        const mainchain = new TestMainchain();
        const mainchainUrl = await mainchain.launch();
        const notary = new TestNotary();
        await notary.start(mainchainUrl);

        const mainchainClient = await getClient(mainchainUrl);

        await notary.register(mainchainClient);
        mainchainClient.disconnect().catch(console.error);
    });
});