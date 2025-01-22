// This function calculates the number of labs needed to research a technology in a given time.
// Where:
//  n       = number of labs
//  level   = current research level
//  m_r     = sum of module effects as %
//  t_r     = research cycle time (this is the time to research a single unit as reported in the research window)
pub fn calculate_research_stats(n: f64, level: f64, m_r: f64, t_r: f64) -> (f64, f64, f64) {
    let ers = (1.0 + level / 100.0) * (1.0 + m_r / 100.0);
    let act = t_r / ers;
    let pps = n / act;
    (ers, act, pps)
}
