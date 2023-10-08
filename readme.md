# Brief
It's an apartment scraper bot. I created it to find the apartment for my grandma who migrated to Kyiv.
This bot requests the particular page on lun.ua, checks for the newly appeared apartments, 
and if there are any new ads emerged, it sends the notification to the Telegram bot.

# Details 
- **Filters**. There are filters set such as "max_price", "streets", "max_floor" in the query string 
of the scraper link to include only appropriate apartments.

- **Check for new ads**. For now, the check for new ads implementation is simple: on a first request 
it initializes the numbers of ads global counter; on subsequent requests if the number of got ads is less than
the global counter value, the global counter just updates its value; if the number of ads is greater than
the global counter value, then it sends the telegram notification that new ads appeared and updates the global
counter value. It may happen, for instance, that between requests one new add appeared and another one disappeared.
In this case we won't get the notification about new ads. To mitigate this problem I set little time to sleep between
requests. Besides, for the current filters the frequency of addition and deletion of ads is low, that's why such
inconsistencies are unlikely.

- **Errors on requests**. If there are any errors occurred during the request to lun.ua, there are set the defined 
amount of retries with sleep time between subsequent request to provide fault tolerance.

- **Telemetry**. To be able to investigate any bugs happen in the production environments, the tracing logging was set.
The file appender was used to persist all logs to the particular folder to be able to easily access it any time.

- **Telegram bot notifications**. For now, the telegram bot sends the notification only to hardcoded user by their chat id.

# Future improvements
- **Check for new ads**. It'll be better to store adds ids got from lun.ua to distinguish apartments more properly to 
resolve some edge cases that were described previously. 

- **Telegram bot**. It'll be better to implement the telegram bot subscription logic to eliminated hardcoding of chat ids,
so that any user with the link to the telegram bot can subscribe to the notifications.

- **Persistence**. To preserve all subscribers and already processed ads on any system failures, it'll be better to use
the full-fledged db.