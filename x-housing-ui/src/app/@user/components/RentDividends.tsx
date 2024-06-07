import { XPTokenAttributes, XProjectSC } from '@/contracts/xProject';
import { useXProjectsInfo } from '@/contracts/xProject/hooks';
import { useAccountTokens, useGetAccount } from '@/hooks';
import { getWindowLocation, prettyFormatAmount } from '@/utils';
import { signAndSendTransactions } from '@/utils/signAndSendTransactions';
import { TokenTransfer } from '@multiversx/sdk-core/out';
import { NonFungibleTokenOfAccountOnNetwork } from '@multiversx/sdk-network-providers/out';
import BigNumber from 'bignumber.js';
import { useCallback } from 'react';
import useSWR from 'swr';

export default function RentDividends() {
  const { address } = useGetAccount();
  const { data } = useXProjectsInfo();
  const { xProjectsToken } = useAccountTokens();
  const { data: tokenBalances } = useSWR(
    data && xProjectsToken
      ? { key: 'projects-rent-rewards', data, tokens: xProjectsToken }
      : null,
    ({ tokens, data }) =>
      Promise.all(
        tokens.map(async (token) => {
          const xProject = data.find(
            ({ tokenId }) => token.collection == tokenId
          );
          const claimable = await xProject?.contract.getRentClaimAble({
            xptAttr: XPTokenAttributes.fromBase64(
              token.attributes.toString('base64')
            )
          });

          return {
            claimable: claimable || new BigNumber(0),
            token,
            contract: xProject?.contract
          };
        })
      )
  );

  const onClaimRentReward = useCallback(
    async (token: NonFungibleTokenOfAccountOnNetwork, contract: XProjectSC) => {
      const tx = contract.makeClaimRentRewardTx({
        address,
        payment: TokenTransfer.metaEsdtFromAmount(
          token.collection,
          token.nonce,
          token.balance,
          0
        )
      });

      return await signAndSendTransactions({
        transactions: [tx],
        callbackRoute: getWindowLocation().pathname,
        transactionsDisplayInfo: {}
      });
    },
    [address]
  );

  return (
    <div className='row pt-2'>
      {tokenBalances?.map(({ claimable, token, contract }) => {
        if (claimable.lte(0) || !contract) {
          return null;
        }

        return (
          <div className='col-6 col-sm-3 col-xxl-2'>
            <a
              onClick={(e) => {
                e.preventDefault();

                onClaimRentReward(token, contract);
              }}
              className='element-box el-tablo centered trend-in-corner smaller'
              href='#'
            >
              <div className='label'>{token.name} Rent Reward</div>
              <div className='value'>
                {prettyFormatAmount({ value: claimable.toFixed(0), length: 4 })}
              </div>
              <div className='trending trending-up'>
                <span>{token.collection}</span>
                <i className='os-icon os-icon-arrow-up6'></i>
              </div>
            </a>
          </div>
        );
      })}
    </div>
  );
}
