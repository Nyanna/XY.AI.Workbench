package xy.ai.conscious.connector.openapi.anthropic;

/** One method per operation; description/example copied from the operation (D-verbindlich). */
public interface V1Client {

    /**
     * Send a structured list of input messages with text and/or image content, and the model will generate the next message in the conversation.
     * 
     * The Messages API can be used for either single queries or stateless multi-turn conversations.
     * 
     * Learn more about the Messages API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.V1MessagesResponse messages_post(xy.ai.conscious.connector.openapi.anthropic.components.CreateMessageParams request);

    /**
     * List all Message Batches within a Workspace. Most recently created batches are returned first.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.get.V1MessagesBatchesResponse message_batches_list(String beforeId, String afterId, Long limit);

    /**
     * Send a batch of Message creation requests.
     * 
     * The Message Batches API can be used to process multiple Messages API requests at once. Once a Message Batch is created, it begins processing immediately. Batches can take up to 24 hours to complete.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.post.V1MessagesBatchesResponse message_batches_post(xy.ai.conscious.connector.openapi.anthropic.request.v1.messages.batches.post.json.CreateMessageBatchParams request);

    /**
     * Delete a Message Batch.
     * 
     * Message Batches can only be deleted once they've finished processing. If you'd like to delete an in-progress batch, you must first cancel it.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.delete.V1MessagesBatchesMessageBatchIdResponse message_batches_delete(String messageBatchId);

    /**
     * This endpoint is idempotent and can be used to poll for Message Batch completion. To access the results of a Message Batch, make a request to the `results_url` field in the response.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.get.V1MessagesBatchesMessageBatchIdResponse message_batches_retrieve(String messageBatchId);

    /**
     * Batches may be canceled any time before processing ends. Once cancellation is initiated, the batch enters a `canceling` state, at which time the system may complete any in-progress, non-interruptible requests before finalizing cancellation.
     * 
     * The number of canceled requests is specified in `request_counts`. To determine which requests were canceled, check the individual results within the batch. Note that cancellation may not result in any canceled requests if they were non-interruptible.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.cancel.V1MessagesBatchesMessageBatchIdCancelResponse message_batches_cancel(String messageBatchId);

    /**
     * Streams the results of a Message Batch as a `.jsonl` file.
     * 
     * Each line in the file is a JSON object containing the result of a single request in the Message Batch. Results are not guaranteed to be in the same order as requests. Use the `custom_id` field to match results to requests.
     * 
     * Learn more about the Message Batches API in our user guide
     */

    xy.ai.conscious.connector.openapi.anthropic.response.v1.messages.batches.message_batch_id.results.V1MessagesBatchesMessageBatchIdResultsResponse message_batches_results(String messageBatchId);
}
