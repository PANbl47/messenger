# Alpha Manual Test Checklist

## Accounts

- [ ] Phone-first sign-up creates an account with username and display name
- [ ] Login-first sign-up creates an account with unique login, password, username, and display name
- [ ] Phone-first account can link login/password later
- [ ] Login-first account can link phone later
- [ ] Email linking returns a masked recovery hint only

## Devices And Recovery

- [ ] First device becomes trusted immediately
- [ ] Second device requires trusted-device approval
- [ ] Device list shows trust state and last active time
- [ ] Recovery enrollment without a trusted device restores account access but does not promise history access
- [ ] Password reset invalidates password-derived recovery material

## Messaging

- [ ] Offline text send stays visible in the timeline
- [ ] Reconnect automatically retries queued text messages
- [ ] After 3 minutes offline, message enters explicit failure state without leaving the timeline
- [ ] Retry from failed state returns the message to queued or delivered based on network state
- [ ] Edit, delete, reply, and forward events stay ordered across subscribers

## Media And Voice

- [ ] Interrupted attachment upload resumes without duplicate logical messages
- [ ] Expired upload tickets renew before final failure
- [ ] Voice draft can be cancelled back into an editable state
- [ ] Storage-full path blocks attachment queueing before pretending success

## Clients

- [ ] Shared web client builds and passes unit tests
- [ ] Shared web client Playwright flow covers offline send, reconnect, and failure timeout
- [ ] Android `testDebugUnitTest` passes
- [ ] Desktop shell `cargo check -p desktop-shell` passes after web dist is built
