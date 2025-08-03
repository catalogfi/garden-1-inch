import { Address, encodeCancelOrder, MakerTraits } from '@1inch/fusion-sdk';
import { FusionApi, QuoterRequest, RelayerRequest, QuoterCustomPresetRequest, ActiveOrdersRequest, OrdersByMakerRequest, OrderStatusRequest } from '../api';
export class SDK {
    constructor(config) {
        this.config = config;
        this.api = new FusionApi({
            url: config.url,
            httpProvider: config.httpProvider,
            authKey: config.authKey
        });
    }
    async getActiveOrders(params = {}) {
        const request = new ActiveOrdersRequest(params);
        return this.api.getActiveOrders(request);
    }
    async getOrderStatus(orderHash) {
        const request = new OrderStatusRequest({ orderHash });
        return this.api.getOrderStatus(request);
    }
    async getOrdersByMaker(params) {
        const request = new OrdersByMakerRequest(params);
        return this.api.getOrdersByMaker(request);
    }
    async getReadyToAcceptSecretFills(orderHash) {
        return this.api.getReadyToAcceptSecretFills(orderHash);
    }
    async getReadyToExecutePublicActions() {
        return this.api.getReadyToExecutePublicActions();
    }
    async getPublishedSecrets(orderHash) {
        return this.api.getPublishedSecrets(orderHash);
    }
    async submitSecret(orderHash, secret) {
        return this.api.submitSecret(orderHash, secret);
    }
    async getQuote(params) {
        const request = new QuoterRequest({
            srcChain: params.srcChainId,
            dstChain: params.dstChainId,
            srcTokenAddress: params.srcTokenAddress,
            dstTokenAddress: params.dstTokenAddress,
            amount: params.amount,
            walletAddress: params.walletAddress || Address.ZERO_ADDRESS.toString(),
            permit: params.permit,
            enableEstimate: !!params.enableEstimate,
            fee: params?.takingFeeBps,
            source: params.source,
            isPermit2: params.isPermit2
        });
        return this.api.getQuote(request);
    }
    async getQuoteWithCustomPreset(params, body) {
        const paramsRequest = new QuoterRequest({
            srcChain: params.srcChainId,
            dstChain: params.dstChainId,
            srcTokenAddress: params.srcTokenAddress,
            dstTokenAddress: params.dstTokenAddress,
            amount: params.amount,
            walletAddress: params.walletAddress || Address.ZERO_ADDRESS.toString(),
            permit: params.permit,
            enableEstimate: !!params.enableEstimate,
            fee: params?.takingFeeBps,
            source: params.source,
            isPermit2: params.isPermit2
        });
        const bodyRequest = new QuoterCustomPresetRequest({
            customPreset: body.customPreset
        });
        return this.api.getQuoteWithCustomPreset(paramsRequest, bodyRequest);
    }
    async createOrder(quote, params) {
        if (!quote.quoteId) {
            throw new Error('request quote with enableEstimate=true');
        }
        const order = quote.createOrder({
            hashLock: params.hashLock,
            receiver: params.receiver
                ? new Address(params.receiver)
                : undefined,
            preset: params.preset,
            nonce: params.nonce,
            takingFeeReceiver: params.fee?.takingFeeReceiver,
            permit: params.permit,
            isPermit2: params.isPermit2
        });
        const hash = order.getOrderHash(quote.srcChainId);
        return { order, hash, quoteId: quote.quoteId };
    }
    async submitOrder(srcChainId, order, quoteId, secretHashes) {
        if (!this.config.blockchainProvider) {
            throw new Error('blockchainProvider has not set to config');
        }
        if (!order.multipleFillsAllowed && secretHashes.length > 1) {
            throw new Error('with disabled multiple fills you provided secretHashes > 1');
        }
        else if (order.multipleFillsAllowed && secretHashes) {
            const secretCount = order.escrowExtension.hashLockInfo.getPartsCount() + 1n;
            if (secretHashes.length !== Number(secretCount)) {
                throw new Error('secretHashes length should be equal to number of secrets');
            }
        }
        const orderStruct = order.build();
        const signature = await this.config.blockchainProvider.signTypedData(orderStruct.maker, order.getTypedData(srcChainId));
        const relayerRequest = new RelayerRequest({
            srcChainId,
            order: orderStruct,
            signature,
            quoteId,
            extension: order.extension.encode(),
            secretHashes: secretHashes.length === 1 ? undefined : secretHashes
        });
        await this.api.submitOrder(relayerRequest);
        return {
            order: orderStruct,
            signature,
            quoteId,
            orderHash: order.getOrderHash(srcChainId),
            extension: relayerRequest.extension
        };
    }
    async placeOrder(quote, params) {
        const { order, quoteId } = await this.createOrder(quote, params);
        return this.submitOrder(quote.srcChainId, order, quoteId, params.secretHashes);
    }
    async buildCancelOrderCallData(orderHash) {
        const getOrderRequest = new OrderStatusRequest({ orderHash });
        const orderData = await this.api.getOrderStatus(getOrderRequest);
        if (!orderData) {
            throw new Error(`Can not get order with the specified orderHash ${orderHash}`);
        }
        const { order } = orderData;
        return encodeCancelOrder(orderHash, new MakerTraits(BigInt(order.makerTraits)));
    }
}
//# sourceMappingURL=sdk.js.map