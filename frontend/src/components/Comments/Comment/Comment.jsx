import React from 'react';
import 'katex/dist/katex.min.css';

export default function Comment({ commentData }) {
    let { author, created_at } = commentData;
    return (
        <div>
            <h2>{author}</h2>
            <p>{created_at}</p>
        </div>
    );
}