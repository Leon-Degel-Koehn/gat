# gat
Git Account Tracker - stop juggling git configs

## Why?
### Easy author management
If you are like me, you oftentimes find yourself managing multiple github/gitlab/... accounts on the same machine.
Inevitably you end up pushing commits that you have accidentally made using the wrong username. Depending on the situation you might be able to reset the author on your commit,
but murphy's law dictates that of course you have made this commit to a branch that doesn't allow force pushing. Great, now everyone can see the amazing changes noobmaster69 has introduced.
Using *gat* you can simply select one of your profiles while you are in a repository and it will make sure the correct author is used for you.

### Manage access tokens
As I am using multiple accounts, some for my school work, others for work and some for my private github, I have a lot of personal access tokens flying around
Now it comes time for me to clone a new repo at work at would you know it, I have no idea which access token to use for that or it is a hassle to search for it on my system.
With *gat* you just simply store your access token as part of one of your profiles and when it comes time to clone a new repo, boom *gat* does it for you.
