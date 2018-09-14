
#include "alloc_btree_internal_header.h"

static struct s_node	*simulate_insert_child(
			struct s_node *node,
			void *content,
			struct s_node_params *use_ctx)
{
	int diff;

	if ((diff = use_ctx->comp(content, node)) < 0) {
		if (node->left == NULL)
			return (node);
		else
			return (simulate_insert_child(
					node->left, content, use_ctx));
	} else if (diff > 0) {
		if (node->right == NULL)
			return (node);
		else
			return (simulate_insert_child(
					node->right, content, use_ctx));
	}
	use_ctx->reg = NODE_ALREADY_PRESENT;
	return (node);
}

static struct s_node	*create_node(
			void *content,
			struct s_node_params *use_ctx)
{
	struct s_node	*new;

	if (use_ctx->associator == NULL || use_ctx->allocator == NULL)
		return (NULL);
	if (!(new = (struct s_node *)use_ctx->allocator(sizeof(struct s_node))))
		return (NULL);
	use_ctx->reg = NODE_ALLOCATED;
	new->left = NULL;
	new->right = NULL;
	use_ctx->associator(content, new);
	return (new);
}

static struct s_node	*alloc_btree_try_insert_node(
			struct s_node **root,
			void *content,
			struct s_node_params *use_ctx)
{
	struct s_node	*candidate;
	struct s_node	*new;

	if (*root == NULL) {
		if ((new = create_node(content, use_ctx)) == NULL)
			return (NULL);
		*root = new;
		new->parent = NULL;
		return (new);
	}

	candidate = simulate_insert_child(*root, content, use_ctx);
	if (use_ctx->reg == NODE_ALREADY_PRESENT)
		return (candidate);
	if ((new = create_node(content, use_ctx)) == NULL)
		return (NULL);
	if (use_ctx->comp(content, candidate) < 0)
		candidate->left = new;
	else
		candidate->right = new;
	new->parent = candidate;
	return (new);
}

struct s_node		*alloc_btree_try_to_insert_rnb_node(
			struct s_node **root,
			void *content,
			struct s_node_params *use_ctx)
{
	struct s_node *new;
	struct s_node *new_root;

	if (root == NULL || use_ctx == NULL || use_ctx->comp == NULL)
		return (NULL);
	use_ctx->reg = ERROR;
	new = alloc_btree_try_insert_node(root, content, use_ctx);
	if (new == NULL)
		return (NULL);
	if (use_ctx->reg == NODE_ALREADY_PRESENT)
		return (new);
	SET_RED(new);
	apply_insert_strategy(new);
	new_root = new;
	while (new_root->parent != NULL)
		new_root = new_root->parent;
	*root = new_root;
	SET_BLACK((*root));
	return (new);
}
