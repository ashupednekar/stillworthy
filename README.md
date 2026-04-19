#StillWorthy 

We are still worthy, just need to be reminded of that

<img width="1080" height="1080" alt="image" src="https://github.com/user-attachments/assets/1fe28845-5016-4c2a-bdfc-0b74490eaccf" />



This tool is meant to help you get sane again cuz vibe coding recently has gotten worse than doomscrolling


## Key goals

- Be practical, not a full deny
- Hard time(per repo/worktree) limit to use agents, period
- Still allow chatgpt/claude chat's but only QnA... L7 filtering/ Clipboard blocking/ custom chat thing, let's see

## Components

**swctl**

The cli that manages:
- let you enable egress proxy... cannot disable without reboot
- TUI to see agent stats (not tokens.. time/questions/etc) 


**app**
App for mac os top bar kinda thing

**swproxy**
The egress proxy that'll be using nftables/ mac equivalent to properly block selectiven traffic... or maybe custom nameserver called from /etc/hosts


