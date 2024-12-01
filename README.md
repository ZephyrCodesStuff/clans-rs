# `clans-rs`

A Rust 🦀 implementation of the Sony Clans API for the PlayStation 3 system.


## Authors

- [@zeph](https://www.github.com/ZephyrCodesStuff)

## Used by

This project is used by the [Destination Home](https://github.com/DestinationHome) team — a revival project for the PlayStation® 3 game "PlayStation Home".

## License
This project is licensed under the GPL-3.0 license. See the [LICENSE](LICENSE) file for more information.

In short,
- You are free to use this project for any purpose.
- You are free to modify this project.
- You are free to distribute this project.
- You are free to distribute modified versions of this project.

However,
- You **must** include the original license in any redistribution.
- You **must** disclose the source code of any modified versions of this project.

## Contributing

Contributions are always welcome!

To get started, simply fork the repo and do your edits, then make a PR to propose your changes, ideally describing them properly in the PR.

## ✅ Implementation status

Below are all of the API endpoints that are available in the Sony Clans API.

The ones that are marked with an "x" are the ones that have been implemented so far.

- `.../func/...`: Normal endpoints
- `.../sec/...`: Secure endpoints, contain a Ticket for authentication.

### Invites and requests

#### Invitations
- [x] `/clan_manager_update/sec/send_invitation`
- [x] `/clan_manager_update/sec/cancel_invitation`
- [x] `/clan_manager_update/sec/accept_invitation`
- [x] `/clan_manager_update/sec/decline_invitation`

#### Membership requests
- [x] `/clan_manager_update/sec/accept_membership_request`
- [x] `/clan_manager_update/sec/decline_membership_request`
- [x] `/clan_manager_update/sec/request_membership`
- [x] `/clan_manager_update/sec/cancel_request_membership`

### Clans

- [ ] `/clan_manager_view/func/clan_search`
- [x] `/clan_manager_view/func/get_clan_info`
- [x] `/clan_manager_view/sec/get_clan_list`
- [ ] `/clan_manager_view/func/get_clan_list_by_jid`

#### Clan management

- [x] `/clan_manager_update/sec/create_clan`
- [x] `/clan_manager_update/sec/update_clan_info`
- [x] `/clan_manager_update/sec/disband_clan`
- [x] `/clan_manager_update/sec/join_clan`
- [x] `/clan_manager_update/sec/leave_clan`

### Members

- [x] `/clan_manager_view/sec/get_member_info`
- [x] `/clan_manager_view/sec/get_member_list`

#### Member management

- [x] `/clan_manager_update/sec/change_member_role`
- [x] `/clan_manager_update/sec/update_member_info`
- [x] `/clan_manager_update/sec/kick_member`

### Blacklist

- [x] `/clan_manager_view/sec/get_blacklist`
- [x] `/clan_manager_update/sec/record_blacklist_entry`
- [x] `/clan_manager_update/sec/delete_blacklist_entry`

### Announcements

- [x] `/clan_manager_view/sec/retrieve_announcements`
- [x] `/clan_manager_update/sec/post_announcement`
- [x] `/clan_manager_update/sec/delete_announcement`

#### Challenge announcements

- [ ] `/clan_manager_view/sec/retrieve_challenge_announcements`
- [ ] `/clan_manager_update/sec/post_challenge_announcement`
- [ ] `/clan_manager_update/sec/delete_challenge_announcement`

#### Posted challenge announcements

- [ ] `/clan_manager_view/sec/retrieve_posted_challenge_announcements`
- [ ] `/clan_manager_update/sec/delete_posted_challenge_announcement`
