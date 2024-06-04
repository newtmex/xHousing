export default function ReferralCard() {
  return (
    <div className='cta-w orange text-center'>
      <div className='cta-content extra-padded'>
        <div className='highlight-header'>Bonus</div>
        <h5 className='cta-header'>
          Invite your friends and make money with referrals
        </h5>
        <form action=''>
          <div className='newsletter-field-w'>
            <input placeholder='Email address...' type='text' />
            <button className='btn btn-sm btn-primary'>Send</button>
          </div>
        </form>
      </div>
    </div>
  );
}
