#! /bin/bash

curl -X POST 0.0.0.0:8080/text/highlight \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d 'text=Info white test message white&api_app_id=any&channel_id=any&channel_name=any&command=any&is_enterprise_install=false&response_url=any&team_domain=any&team_id=any&token=any&trigger_id=any&user_id=any&user_name=any'

echo -e "\n"

curl -X POST 0.0.0.0:8080/text/highlight \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d 'text=Warn yellow test message yellow&api_app_id=any&channel_id=any&channel_name=any&command=any&is_enterprise_install=false&response_url=any&team_domain=any&team_id=any&token=any&trigger_id=any&user_id=any&user_name=any'

echo -e "\n"

curl -X GET 0.0.0.0:8080/
