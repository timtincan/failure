TroyLabs initially started as the Trojan Venture Fund which was a student-run venture capital fund.

Imagine you’re a VC fund and you have a list of investments in companies with certain dollar values. The investments are as follows:

1. Buy Apple - 40
2. Buy SoundMind - 50
3. Buy Robinhood - 30
4. Buy Inked Sports - 120
5. Sell Apple -  40
6. Buy Google - 10

The fund has $100 to start. When a stake is purchased in a company that leads to all of the funds being exhausted, some of the stake in the MOST RECENTLY invested company (think of this like a margin call) should be sold off to free up the funds.

NOTE: This liquidation can be cascading. That is, you may have to liquidate multiple stakes to buy the new one. Do this in order of MOST RECENTLY invested company.

Write a function that takes in the above data (see below for the input format), and outputs the final portfolio of the VC (invested companies and stake in each) as well as remaining cash. In the above example, this would be:

1. SoundMind - 30
2. Robinhood - 30
3. Google - 10
4. Cash - 30

Note that when we invest in Robinhood, we have then invested 120. As a result, we sold off 20 of our stake in SoundMind to make that investment.

Also note that Inked Sports was a failed transaction and nothing happened. This is because the fund doesn’t have 120 to invest in the first place, so we didn’t allow that to go through.

Be sure to think about edge cases such as investing too much (more than the fund even has available).

**Stretch Goal**

Upgrade your code to track invalid transactions and output them to the user along with the final portfolio.

**Stretch Goal 2**

Upgrade your code to output some “transaction logs” to the user, describing what happened at each step.

**Test Case 1:**

BUY, AAPL, 40

BUY, SNMD, 50

BUY, HOOD, 30

BUY, Inked Sports, 120

SELL, AAPL, 40

BUY, Google, 10

Expect:

SNMD, 30

HOOD, 30

GOOGL, 10

CASH, 30