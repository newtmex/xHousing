import { ReturnCode } from '@multiversx/sdk-core/out';

export class XHousingnNetworkError extends Error {
  code: ReturnCode;

  constructor(
    readonly errorBag: {
      code: ReturnCode;
      message: string;
    }
  ) {
    super(errorBag.message);
    this.code = errorBag.code;
  }
}
