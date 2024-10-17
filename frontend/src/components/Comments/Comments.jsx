import React, { useState, useEffect } from 'react';
import Comment from './Comment/Comment.jsx';
import { useSelector, useDispatch } from 'react-redux';
import { commentsGet } from '../../store/comments.js';

export default function Comments() {
  const dispatch = useDispatch();
  const { comments } = useSelector((state) => state);
  const [showComments, setShowComments] = useState(false);

  useEffect(() => {
    dispatch(commentsGet());
  }, [dispatch]);

  const toggleShowComments = () => {
    setShowComments((prevShowComments) => !prevShowComments);
  };

  return (
    <div>
      <div className="flex">
        <h1 className="flex-1">COMMENTS</h1>
        <button onClick={toggleShowComments}>
          {showComments ? 'Hide Comments' : 'Show Comments'}
        </button>
      </div>
      {showComments && comments.map((commentData, index) => (
        <Comment commentData={commentData} key={index} />
      ))}
    </div>
  );
}