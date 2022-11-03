#[derive(Clone, Debug)]
enum Cluster {
    Cluster(Box<Cluster>, Box<Cluster>, f64),
    Leaf(usize),
}

impl Cluster {
    fn get_indices(&self) -> Vec<usize> {
        match self {
            Cluster::Cluster(c1, c2, _) => {
                let mut indices = c1.get_indices();
                indices.extend(c2.get_indices());
                indices
            }
            Cluster::Leaf(index) => vec![*index],
        }
    }

    fn len(&self) -> usize {
        match self {
            Cluster::Cluster(c1, c2, _) => c1.len() + c2.len(),
            Cluster::Leaf(_) => 1,
        }
    }

    fn get_lines(&self) -> Vec<String> {
        match self {
            Cluster::Cluster(c1, c2, age) => {
                let subtree1 = c1.get_lines();
                let subtree2 = c2.get_lines();

                let max_subtree_length = std::cmp::max(
                    subtree1.iter().map(|x| x.chars().count()).max(),
                    subtree2.iter().map(|x| x.chars().count()).max(),
                )
                .unwrap();

                let label = age.to_string();
                let label_len = label.len();

                let mut tree_lines: Vec<String> = subtree1
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        if i == 0 {
                            format!("{}─{:─>max_subtree_length$}", label, line)
                        } else {
                            format!("{:<label_len$} {:>max_subtree_length$}", "│", line)
                        }
                    })
                    .collect();
                tree_lines.extend(subtree2.iter().enumerate().map(|(i, line)| {
                    if i == 0 {
                        format!("{:─<label_len$}─{:─>max_subtree_length$}", "└", line)
                    } else {
                        format!("{} {:>max_subtree_length$}", " ".repeat(label.len()), line)
                    }
                }));

                tree_lines
            }
            Cluster::Leaf(node) => {
                vec![match node {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    3 => "D",
                    _ => panic!(),
                }
                .to_owned()]
            }
        }
    }
}

fn main() {
    let distance_matrix = vec![
        vec![0, 2, 4, 6],
        vec![2, 0, 4, 6],
        vec![4, 4, 0, 6],
        vec![6, 6, 6, 0],
    ];

    let mut clusters: Vec<Cluster> = (0..4).map(|x| Cluster::Leaf(x)).collect();

    while clusters.len() > 1 {
        let mut min_pair: Option<(usize, usize)> = None;
        let mut min_dist = std::f64::MAX;

        for i in 0..clusters.len() {
            let c1 = &clusters[i];
            for j in (i + 1)..clusters.len() {
                let c2 = &clusters[j];
                let dist = dist_avg(&distance_matrix, &c1, &c2);
                if dist < min_dist {
                    min_dist = dist;
                    min_pair = Some((i, j));
                }
            }
        }

        let (i, j) = min_pair.unwrap();
        let c2 = clusters.remove(j);
        let c1 = clusters.remove(i);

        let new_cluster = Cluster::Cluster(Box::new(c1), Box::new(c2), min_dist / 2.0);
        clusters.push(new_cluster);
    }

    let cluster = clusters.first().unwrap();

    for line in cluster.get_lines() {
        println!("{}", line);
    }
}

fn dist_avg(d: &Vec<Vec<i32>>, c1: &Cluster, c2: &Cluster) -> f64 {
    let mut total = 0;

    for i in c1.get_indices() {
        for j in c2.get_indices() {
            total += d[i][j];
        }
    }

    total as f64 / (c1.len() * c2.len()) as f64
}
