import React from "react";
import { useQuery, gql } from '@apollo/client';

const USER_INFO = gql`
query GetUserInfo {
    user(id: "1234") {
        username
        permissionGroups
        accountId
    }
    
    account(id:"123") {
        id
        name
        dateJoined
    }
}
`;

function Header() {
    const { loading, error, data } = useQuery(USER_INFO);

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error :(</p>;

    return (<p>
        {data.user.username}
    </p>)
}

export default Header;