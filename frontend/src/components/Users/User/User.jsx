
import React from 'react';
import 'katex/dist/katex.min.css';

export default function User({ userData }) {
    let { username, created_at } = userData;
    return (
        <div>
            <h2>{username}</h2>
            <p>{created_at}</p>
        </div>
    );
}