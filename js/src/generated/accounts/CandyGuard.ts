/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js';
import * as beetSolana from '@metaplex-foundation/beet-solana';
import * as beet from '@metaplex-foundation/beet';

/**
 * Arguments used to create {@link CandyGuard}
 * @category Accounts
 * @category generated
 */
export type CandyGuardArgs = {
  base: web3.PublicKey;
  bump: number;
  authority: web3.PublicKey;
};

export const candyGuardDiscriminator = [44, 207, 199, 184, 112, 103, 34, 181];
/**
 * Holds the data for the {@link CandyGuard} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class CandyGuard implements CandyGuardArgs {
  private constructor(
    readonly base: web3.PublicKey,
    readonly bump: number,
    readonly authority: web3.PublicKey,
  ) {}

  /**
   * Creates a {@link CandyGuard} instance from the provided args.
   */
  static fromArgs(args: CandyGuardArgs) {
    return new CandyGuard(args.base, args.bump, args.authority);
  }

  /**
   * Deserializes the {@link CandyGuard} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(accountInfo: web3.AccountInfo<Buffer>, offset = 0): [CandyGuard, number] {
    return CandyGuard.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link CandyGuard} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
  ): Promise<CandyGuard> {
    const accountInfo = await connection.getAccountInfo(address);
    if (accountInfo == null) {
      throw new Error(`Unable to find CandyGuard account at ${address}`);
    }
    return CandyGuard.fromAccountInfo(accountInfo, 0)[0];
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey('FhCHXHuD6r2iCGwHgqcgnDbwXprLf22pZcArSp4Si4n7'),
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, candyGuardBeet);
  }

  /**
   * Deserializes the {@link CandyGuard} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [CandyGuard, number] {
    return candyGuardBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link CandyGuard} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return candyGuardBeet.serialize({
      accountDiscriminator: candyGuardDiscriminator,
      ...this,
    });
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link CandyGuard}
   */
  static get byteSize() {
    return candyGuardBeet.byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link CandyGuard} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(CandyGuard.byteSize, commitment);
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link CandyGuard} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === CandyGuard.byteSize;
  }

  /**
   * Returns a readable version of {@link CandyGuard} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      base: this.base.toBase58(),
      bump: this.bump,
      authority: this.authority.toBase58(),
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const candyGuardBeet = new beet.BeetStruct<
  CandyGuard,
  CandyGuardArgs & {
    accountDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['base', beetSolana.publicKey],
    ['bump', beet.u8],
    ['authority', beetSolana.publicKey],
  ],
  CandyGuard.fromArgs,
  'CandyGuard',
);
