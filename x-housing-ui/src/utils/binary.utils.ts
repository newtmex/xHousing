import { Address } from '@multiversx/sdk-core/out';
import { Hash } from '@multiversx/sdk-core/out/hash';
// import { AddressUtils } from "./address.utils";

function base64DecodeBinary(str: string): Buffer {
  return Buffer.from(str, 'base64');
}

export class BinaryUtils {
  static base64Encode(str: string) {
    return Buffer.from(str).toString('base64');
  }

  static base64Decode(str: string): string {
    return base64DecodeBinary(str).toString('binary');
  }

  static tryBase64ToBigInt(str: string): bigint | undefined {
    try {
      return this.base64ToBigInt(str);
    } catch {
      return undefined;
    }
  }

  static base64ToBigInt(str: string): bigint {
    return BigInt('0x' + this.base64ToHex(str));
  }

  static tryBase64ToHex(str: string): string | undefined {
    try {
      return this.base64ToHex(str);
    } catch {
      return undefined;
    }
  }

  static base64ToHex(str: string): string {
    return Buffer.from(str, 'base64').toString('hex');
  }

  static stringToHex(str: string): string {
    return Buffer.from(str).toString('hex');
  }

  static tryBase64ToAddress(str: string): string | undefined {
    try {
      return this.base64ToAddress(str);
    } catch {
      return undefined;
    }
  }

  static base64ToAddress(str: string): string {
    return Address.fromHex(this.base64ToHex(str)).bech32();
  }

  static hexToString(hex: string): string {
    return Buffer.from(hex, 'hex').toString('ascii');
  }

  static hexToNumber(hex: string): number {
    return parseInt(hex, 16);
  }

  static hexToBigInt(hex: string): bigint {
    if (!hex) {
      return BigInt(0);
    }

    return BigInt('0x' + hex);
  }

  static padHex(value: string): string {
    return value.length % 2 ? '0' + value : value;
  }

  static isHash(value: string): boolean {
    try {
      new Hash(value);
      return true;
    } catch (error) {
      return false;
    }
  }

  static numberToHex(value: number): string {
    return BinaryUtils.padHex(value.toString(16));
  }
}
