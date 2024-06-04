import { PropsWithChildren } from 'react';

export default function ElementWrapper({
  children,
  title
}: PropsWithChildren<{ title: string }>) {
  return (
    <div className='element-wrapper compact'>
      <div className='element-actions actions-only'>
        <a className='element-action element-action-fold' href='#'>
          <i className='os-icon os-icon-minus-circle'></i>
        </a>
      </div>
      <h6 className='element-header'>{title}</h6>
      <div className='element-box-tp'>{children}</div>
    </div>
  );
}
