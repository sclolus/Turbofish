
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>
#include <stdbool.h>

# define BUFF_SIZE 		128
# define MAX_DESCRIPTORS	65536

typedef struct		s_buffer
{
	int			fd;
	int			buff_size;
	size_t			l_size;
	char			buffer[BUFF_SIZE + 1];
} t_buffer;

struct			s_custom_memory_fn
{
	void			*(*allocator)(size_t);
	void			(*deallocator)(void *);
};

static char		*s_concat(
				char **str,
				t_buffer *index,
				size_t n,
				struct s_custom_memory_fn *mem)
{
	char			*output;

	if (!(output = (char *)
		mem->allocator((index->l_size + n + 1) * sizeof(char))))
		return (NULL);
	output[index->l_size + n] = '\0';
	memcpy(output, *str, index->l_size);
	if (index->l_size)
		mem->deallocator(*str);
	*str = output;
	memcpy(*str + index->l_size, index->buffer, n);
	index->l_size += n;
	return (output);
}

static int		s_exec(
				t_buffer *index,
				char **line,
				struct s_custom_memory_fn *mem)
{
	char			*jump_location;
	size_t			i;

	*line = NULL;
	index->l_size = 0;
	while (true) {
		if ((index->buff_size < 1) &&
		(index->buff_size = read(
				index->fd,
				index->buffer,
				BUFF_SIZE)) <= 0)
			return ((index->buff_size == 0 && *line)
					? 1 : index->buff_size);
		index->buffer[index->buff_size] = '\0';
		if ((jump_location = strchr(index->buffer, '\n')))
			break ;
		if (!s_concat(line, index, index->buff_size, mem))
			return EXIT_FAILURE;
		*index->buffer = '\0';
		index->buff_size = 0;
	}
	if (!s_concat(line, index, (i = jump_location - index->buffer), mem))
		return EXIT_FAILURE;
	memmove(index->buffer, jump_location + 1, BUFF_SIZE - (i + 1));
	index->buffer[(index->buff_size -= i + 1)] = '\0';
	return (1);
}

int			get_next_line(
				const int fd,
				char **line,
				struct s_custom_memory_fn *mem)
{
	static t_buffer		*index[MAX_DESCRIPTORS];
	int			i;

	if (fd < 0 || fd == 1 || fd == 2 || !line || !mem ||
			!mem->allocator || !mem->deallocator)
		return EXIT_FAILURE;
	i = 0;
	while (index[i] != NULL && index[i]->fd != fd && i < MAX_DESCRIPTORS)
		i++;
	if (i == MAX_DESCRIPTORS)
		return EXIT_FAILURE;
	if (index[i] == NULL) {
		if (!(index[i] = (t_buffer *)mem->allocator(sizeof(t_buffer))))
			return EXIT_FAILURE;
		bzero(index[i]->buffer, BUFF_SIZE + 1);
		index[i]->buff_size = 0;
		index[i]->fd = fd;
	}
	return (s_exec(index[i], line, mem));
}

char			**create_tab(char *s, int *n_words)
{
	char			**tab;

	while (*s)
	{
		if (*s != ' ' && *s != '\t' && *s != '\n')
		{
			*n_words += 1;
			while (*s && *s != ' ' && *s != '\t' && *s != '\n')
				s++;
			while (*s && (*s == ' ' || *s == '\t' || *s == '\n'))
				s++;
		}
		else
			s++;
	}
	if (!(tab = (char **)malloc((*n_words + 1) * sizeof(char *))))
		return (NULL);
	tab[*n_words] = NULL;
	return (tab);
}

void			ft_strncpy(char *dst, char *src, int n)
{
	while (n--)
		*dst++ = *src++;
}

char			**fill_tab(char *s, char **tab)
{
	char			*ptr;
	int			i;
	int			j;

	i = 0;
	while (*s)
	{
		if (*s != ' ' && *s != '\t' && *s != '\n')
		{
			ptr = s;
			while (*s && *s != ' ' && *s != '\t' && *s != '\n')
				s++;
			j = s - ptr;
			if (!(tab[i] = (char *)malloc((j + 1) * sizeof(char))))
				return (NULL);
			tab[i][j] = '\0';
			ft_strncpy(tab[i++], ptr, j);
			while (*s && (*s == ' ' || *s == '\t' || *s == '\n'))
				s++;
		}
		else
			s++;
	}
	return (tab);
}

char			**ft_split_whitespaces(char *str)
{
	char			**tab;
	int			n_words;

	n_words = 0;
	if (!(tab = create_tab(str, &n_words)))
		return (NULL);
	return (fill_tab(str, tab));
}

struct 			s_list
{
	void		*content;
	size_t		content_size;
	struct s_list	*next;
};

struct s_list		*lst_create_elem(void *data, size_t len,
				void *(*allocator)(size_t))
{
	struct s_list		*elmt;

	if (!(elmt = (struct s_list *)allocator(sizeof(struct s_list))))
		return (NULL);
	elmt->content = data;
	elmt->content_size = len;
	return (elmt);
}

struct s_list		*lst_push_back(
				struct s_list **alst,
				void *data,
				size_t len,
				void *(*allocator)(size_t))
{
	struct s_list		*m;
	struct s_list		*ptr;

	if (!(m = lst_create_elem(data, len, allocator)))
		return (NULL);
	if (!(*alst)) {
		*alst = m;
		(*alst)->next = NULL;
		return (*alst);
	}
	ptr = *alst;
	while (ptr->next)
		ptr = ptr->next;
	ptr->next = m;
	m->next = NULL;
	return (*alst);
}

void			lst_del(
				struct s_list **alst,
				void (*del)(void *, size_t, void (*)(void *)),
				void (*deallocator)(void *))
{
	struct s_list		*current;
	struct s_list		*tmp;

	current = *alst;
	while (current) {
		del(current->content, current->content_size, deallocator);
		tmp = current;
		current = current->next;
		deallocator(tmp);
	}
	*alst = NULL;
}

void			delete_list(
				void *s, size_t size,
				void (*unalocator)(void *))
{
	(void)size;
	unalocator(s);
}

int 			write_line(int fd, const char *s)
{
	size_t 			size;

	size = strlen(s);

	return write(fd, s, size);
}

int			main(int argc, char *argv[])
{
	int			fd_file_map;
	int			fd_nm;
	char			*buf;
	struct s_custom_memory_fn mem;
	char			**tab;
	char			final_buf[256];
	struct s_list		*lst;
	int			size;
	size_t			nb_lines;
	struct s_list		*tmp;

	(void)argc;
	(void)argv;

	if (argc > 3 || argc == 1) {
		printf("bad argument number\n");
		return EXIT_FAILURE;
	}

	if (argc == 3) {
		fd_nm = open(argv[2], O_RDONLY);
		if (fd_nm < 0) {
			printf("cannot open %s\n", argv[2]);
			fd_nm = -1;
		}
	} else {
		fd_nm = -1;
	}

	fd_file_map = open(argv[1], O_WRONLY | O_CREAT | O_TRUNC, 00644);
	if (fd_file_map < 0) {
		printf("cannot open %s\n", argv[1]);
		return EXIT_FAILURE;
	}

	mem.allocator = &malloc;
	mem.deallocator = &free;

	lst = NULL;
	buf = NULL;
	nb_lines = 0;
	if (fd_nm != -1) {
		while ((size = get_next_line(fd_nm, &buf, &mem)) > 0) {
			if (lst_push_back(&lst, buf, size + 1, &malloc) == NULL) {
				printf("Cannot push back\n");
				return EXIT_FAILURE;
			}
			nb_lines++;
		}
	}

	sprintf(final_buf, "#define FN_DIR_LEN	%lu\n\n", nb_lines);
	write_line(fd_file_map, final_buf);

	write_line(fd_file_map, "static struct symbol_entry "
			"function_directory[FN_DIR_LEN] = {\n");

	tmp = lst;
	while (tmp) {
		tab = ft_split_whitespaces((char *)tmp->content);
		if (tab == NULL) {
			printf("Cannot split whitespace\n");
			return EXIT_FAILURE;
		}

		int i = 0;
		while (tab[i])
			i++;
		if (i == 3) {
			sprintf(final_buf, "\t{0x%s, '%s', \"%s\"},\n",
					tab[0], tab[1], tab[2]);
			write_line(fd_file_map, final_buf);
		} else if (i == 2){
			sprintf(final_buf, "\t{0x%s, '%s', \"%s\"},\n",
					tab[0], tab[1], "");
			write_line(fd_file_map, final_buf);
		} else {
			printf("bad size array\n");
		}

		while (i--)
			free(tab[i]);
		free(tab);
		tmp = tmp->next;
	}

	write_line(fd_file_map, "};\n");

	lst_del(&lst, &delete_list, &free);
	close(fd_file_map);

	if (fd_nm != EXIT_FAILURE)
		close(fd_nm);
	return EXIT_SUCCESS;
}
