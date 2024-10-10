import React, { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';
import 'katex/dist/katex.min.css';
import remarkGfm from 'remark-gfm';
import { useSelector } from 'react-redux';
import * as api from '../../../api';

export default function Post({ postData }) {
  let {post, comments} = postData;
  const user = useSelector((state) => state.user?.result);
  const [isCommenting, setIsCommenting] = useState(true);
  const [comment, setComment] = useState('');

  const handleCommentSubmit = async () => {
    let commentData = {content: comment};
    console.log(user);
    commentData.post_id = post.id;
    commentData.user_id = user.id;
    console.log(commentData);
    await api.addComment({commentData});
    setIsCommenting(false); // Hide the comment input after submitting
  };

  return (
    <div className='m-2 text-clip border-2 border-pink bg-stone-200'>
      <h1 className='text-stone-1000 bg-yellow-500'>{post?.user?.username}</h1>
      <h1 className='text-stone-1000 bg-pink-500'>{post?.title}</h1>
      <div className="overflow-y-scroll">
        <ReactMarkdown
          className='p-2'
          remarkPlugins={[remarkMath, remarkGfm]}
          rehypePlugins={[rehypeKatex]}
        >
          {post.markdown}
        </ReactMarkdown>
      </div>
      
      <div className='border-2 p-2'>
        {comments.map(({comment, user}, index) => (<>
          <h3>{user.username}</h3>
          <h2>{comment}</h2>
        </>))}
      </div>

      {(user && !isCommenting) && (
        <button onClick={() => setIsCommenting(true)}>Add Comment</button>
      )}

      {isCommenting && (
        <div>
          <textarea
            value={comment}
            onChange={(e) => setComment(e.target.value)}
            className="border border-gray-300 p-2 w-full"
            placeholder="Write your comment..."
          />
          <button onClick={handleCommentSubmit} className="mt-2 bg-blue-500 text-white p-2">
            Submit Comment
          </button>
          <button onClick={() => setIsCommenting(false)} className="ml-2 bg-red-500 text-white p-2">
            Cancel
          </button>
        </div>
        )}
    </div>
  );
}